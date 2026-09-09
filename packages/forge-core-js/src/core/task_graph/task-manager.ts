import { Task, TaskStatus, SessionId, TaskId, AgentId, TeamId } from "../types.js";
import { EventBus, createEvent } from "../events/event-bus.js";

export class TaskGraphManager {
  private tasks = new Map<TaskId, Task>();
  private sessionTasks = new Map<SessionId, Set<TaskId>>();

  constructor(private eventBus: EventBus) {}

  createTask(params: {
    session_id: SessionId;
    title: string;
    description?: string;
    depends_on?: TaskId[];
    priority?: number;
    owner?: AgentId;
    team_id?: TeamId;
  }): Task {
    const now = new Date().toISOString();
    const task: Task = {
      id: crypto.randomUUID(),
      session_id: params.session_id,
      title: params.title,
      description: params.description,
      status: TaskStatus.Pending,
      priority: params.priority || 1,
      dependencies: params.depends_on || [],
      owner: params.owner,
      team_id: params.team_id,
      progress: 0,
      created_at: now,
      artifacts: [],
      errors: [],
    };

    this.tasks.set(task.id, task);
    if (!this.sessionTasks.has(task.session_id)) {
      this.sessionTasks.set(task.session_id, new Set());
    }
    this.sessionTasks.get(task.session_id)!.add(task.id);

    this.eventBus.publish(createEvent(task.session_id, {
      type: "task.created",
      data: { task_id: task.id, title: task.title }
    }));

    return task;
  }

  getTask(id: TaskId): Task | undefined {
    return this.tasks.get(id);
  }

  listTasks(sessionId: SessionId): Task[] {
    const ids = this.sessionTasks.get(sessionId);
    if (!ids) return [];
    return Array.from(ids).map(id => this.tasks.get(id)!).filter(Boolean).sort((a, b) => b.priority - a.priority);
  }

  updateTaskStatus(taskId: TaskId, status: TaskStatus, progress?: number): Task | undefined {
    const task = this.tasks.get(taskId);
    if (task) {
      task.status = status;
      if (progress !== undefined) task.progress = progress;
      if (status === TaskStatus.Running && !task.started_at) {
        task.started_at = new Date().toISOString();
        this.eventBus.publish(createEvent(task.session_id, {
          type: "task.started",
          data: { task_id: taskId, agent_id: task.owner }
        }));
      }
      if (status === TaskStatus.Completed) {
        task.completed_at = new Date().toISOString();
        task.progress = 100;
        this.eventBus.publish(createEvent(task.session_id, {
          type: "task.completed",
          data: { task_id: taskId }
        }));
      }
      if (status === TaskStatus.Failed) {
        task.completed_at = new Date().toISOString();
        this.eventBus.publish(createEvent(task.session_id, {
          type: "task.failed",
          data: { task_id: taskId, error: task.errors[task.errors.length - 1] || "Failed" }
        }));
      }
      this.eventBus.publish(createEvent(task.session_id, {
        type: "task.updated",
        data: { task_id: taskId, progress: task.progress, status: task.status }
      }));
    }
    return task;
  }

  assignTask(taskId: TaskId, agentId: AgentId): Task | undefined {
    const task = this.tasks.get(taskId);
    if (task) {
      task.owner = agentId;
    }
    return task;
  }

  getReadyTasks(sessionId: SessionId): Task[] {
    const all = this.listTasks(sessionId);
    const completed = new Set(all.filter(t => t.status === TaskStatus.Completed).map(t => t.id));
    return all.filter(t => 
      t.status === TaskStatus.Pending &&
      t.dependencies.every(dep => completed.has(dep))
    );
  }

  getBlockedTasks(sessionId: SessionId): Task[] {
    return this.listTasks(sessionId).filter(t => t.status === TaskStatus.Blocked);
  }

  buildGraph(sessionId: SessionId) {
    const tasks = this.listTasks(sessionId);
    const edges: [TaskId, TaskId][] = [];
    for (const task of tasks) {
      for (const dep of task.dependencies) {
        edges.push([dep, task.id]);
      }
    }
    return { tasks, edges };
  }
}
