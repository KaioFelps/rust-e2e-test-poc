import clsx from "clsx";
import type { PropsWithChildren } from "react";

type AlertProps = {
  theme: "warn" | "success";
};

export function Alert({ theme, children }: PropsWithChildren<AlertProps>) {
  return (
    <span
      className={clsx(
        "block mb-6 px-12 py-6 text-lg font-medium",
        theme === "warn" && "bg-yellow-300",
        theme === "success" && "bg-green-400",
      )}
    >
      {children}
    </span>
  );
}
