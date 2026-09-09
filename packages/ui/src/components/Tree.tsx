import React from 'react';

interface TreeNode {
  name: string;
  path: string;
  is_dir: boolean;
  children?: TreeNode[];
}

export function Tree({ nodes, onSelect }: { nodes: TreeNode[]; onSelect?: (path: string) => void }) {
  const renderNode = (node: TreeNode, depth = 0) => (
    <div key={node.path}>
      <div
        onClick={() => onSelect?.(node.path)}
        style={{ padding: '4px 8px', paddingLeft: `${8 + depth * 16}px`, fontSize: '12px', cursor: 'pointer', display: 'flex', gap: '6px', color: node.is_dir ? '#e5e5e5' : '#888' }}
      >
        <span>{node.is_dir ? '▸' : '•'}</span>
        <span>{node.name}</span>
      </div>
      {node.children?.map(child => renderNode(child, depth + 1))}
    </div>
  );

  return <div>{nodes.map(n => renderNode(n))}</div>;
}
