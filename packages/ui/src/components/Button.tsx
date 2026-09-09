import React from 'react';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
}

export function Button({ variant = 'secondary', size = 'md', style, children, ...props }: ButtonProps) {
  const base: React.CSSProperties = {
    fontFamily: 'inherit',
    fontSize: size === 'sm' ? '11px' : size === 'lg' ? '13px' : '12px',
    padding: size === 'sm' ? '4px 8px' : size === 'lg' ? '8px 16px' : '6px 12px',
    border: '1px solid #1f1f1f',
    background: variant === 'primary' ? '#fff' : variant === 'ghost' ? 'transparent' : '#111',
    color: variant === 'primary' ? '#000' : '#e5e5e5',
    cursor: 'pointer',
    borderRadius: '4px',
    fontWeight: 500,
  };

  return <button style={{ ...base, ...style }} {...props}>{children}</button>;
}
