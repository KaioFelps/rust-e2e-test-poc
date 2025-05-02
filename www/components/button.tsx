import { Slot } from "@radix-ui/react-slot";
import clsx from "clsx";
import { type ComponentPropsWithoutRef, useMemo } from "react";

type ButtonProps = {
  size?: "lg" | "md";
  theme?: "success" | "default";
  asChild?: boolean;
  alwaysEnabled?: boolean;
} & ComponentPropsWithoutRef<"button">;

export function Button({
  size = "md",
  theme = "default",
  children,
  className,
  asChild = false,
  alwaysEnabled = false,
  ...props
}: ButtonProps) {
  const Element = useMemo(() => (asChild ? Slot : "button"), [asChild]);

  return (
    <Element
      {...props}
      className={clsx(
        ////////////////////////////////////////////////////////////
        // DEFAULT
        ////////////////////////////////////////////////////////////
        "transition-all relative ring-inset ring-0 outline-none",
        ////////////////////////////////////////////////////////////
        // SIZES
        ////////////////////////////////////////////////////////////
        resolveClassNames(
          size === "md",
          [
            "enabled:-top-1 enabled:-left-1 px-6 py-4",
            "text-lg font-bold text-shadow-[3px_3px_0] ",
            "enabled:hover:-top-2 enabled:hover:-left-2 enabled:hover:shadow-[6px_6px_0]",
            "enabled:active:top-0 enabled:active:left-0 enabled:active:shadow-[2px_2px_0]",
            "focus:ring-4",
          ],
          alwaysEnabled,
        ),
        resolveClassNames(
          size === "lg",
          [
            "font-black text-3xl px-8 py-4 shadow-[6px_6px_0] text-shadow-[4px_4px_0]",
            "left-0 top-0",
            "hover:-left-1 hover:-top-1 hover:shadow-[10px_10px_0]",
            "active:top-1 active:left-1 active:shadow-[2px_2px_0]",
            "focus:ring-4",
          ],
          alwaysEnabled,
        ),
        ////////////////////////////////////////////////////////////
        // THEMES
        ////////////////////////////////////////////////////////////
        resolveClassNames(
          theme === "default",
          [
            "text-white bg-sky-500 text-shadow-blue-700 shadow-[4px_4px_0] shadow-black",
            "enabled:hover:shadow-blue-800 enabled:active:bg-sky-600 ",
            "disabled:bg-gray-400 disabled:shadow-[2px_2px_0] ring-blue-300 ",
          ],
          alwaysEnabled,
        ),
        resolveClassNames(
          theme === "success",
          [
            "enabled:bg-green-600 text-white shadow-black text-shadow-green-800",
            "enabled:hover:shadow-green-900 enabled:active:shadow-black enabled:active:bg-green-700",
            "disabled:bg-gray-400 disabled:shadow-[2px_2px_0] disabled:text-shadow-black ring-green-400",
          ],
          alwaysEnabled,
        ),
        className && className,
      )}
    >
      {children}
    </Element>
  );
}

function resolveClassNames(
  condition: boolean,
  classes: string[],
  stripDisabledAndEnabledStatements: boolean,
): string | null {
  if (!condition) return null;
  let classNames = classes.join(" ");

  if (stripDisabledAndEnabledStatements)
    classNames = classNames.replaceAll(/(?:enabled:|disabled:)/g, "");

  return classNames;
}
