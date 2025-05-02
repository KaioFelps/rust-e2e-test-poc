import clsx from "clsx";
import type { FormHTMLAttributes, PropsWithChildren } from "react";

export function Root({
  children,
  className,
  ...props
}: FormHTMLAttributes<HTMLFormElement>) {
  return (
    <form
      {...props}
      className={clsx(
        "p-8 bg-white border-2 border-black flex flex-col gap-5",
        className && className,
      )}
    >
      {children}
    </form>
  );
}
