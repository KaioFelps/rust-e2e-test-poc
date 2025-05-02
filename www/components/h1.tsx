import clsx from "clsx";
import type { PropsWithChildren } from "react";

type H1Props = {
  className?: string;
};

export function H1({ children, className }: PropsWithChildren<H1Props>) {
  return (
    <h1
      className={clsx(
        "font-black text-8xl text-purple-500",
        className && className,
      )}
    >
      {children}
    </h1>
  );
}
