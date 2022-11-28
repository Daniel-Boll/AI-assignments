import React from 'react';
import type { FC, ReactElement, ReactNode } from 'react';
import { clsx } from 'clsx';

interface DrawerProps {
  trigger: ReactElement;
  children?: ReactNode;
}

const Drawer: FC<DrawerProps> = ({ trigger, children }) => {
  return (
    <div className="drawer">
      <input id="drawer-hook" type="checkbox" className="drawer-toggle" />
      <div className="drawer-content">
        {React.cloneElement(trigger as React.ReactElement, {
          className: clsx(trigger.props.className, 'drawer-button'),
          htmlFor: 'drawer-hook',
        })}
      </div>
      <div className="drawer-side">
        <label htmlFor="drawer-hook" className="drawer-overlay" />

        {children}
      </div>
    </div>
  );
};

export { Drawer };
