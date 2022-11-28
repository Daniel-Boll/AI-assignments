import React from "react";

type FCWithChildren<T> = React.FC<T> & {
  children?: React.ReactNode;
};

export { FCWithChildren };
