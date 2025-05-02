import type { ToString } from "@/types/interfaces/to-string";
import { Slot, type SlotProps } from "@radix-ui/react-slot";
import clsx from "clsx";
import type { PropsWithChildren } from "react";

export type InputBaseProps<T> = {
  label: string;
  error?: string;
};

export function InputBase<T extends ToString>({
  label,
  error,
  children,
}: PropsWithChildren<InputBaseProps<T>>) {
  return (
    // biome-ignore lint/a11y/noLabelWithoutControl: <explanation>
    <label>
      <span className="block mb-3 font-black text-2xl text-gray-500">
        {label}
      </span>

      {error && (
        <span className="block mb-2 text-white bg-red-500 px-4 py-2 font-bold text-lg">
          {error}
        </span>
      )}

      <Slot
        className={clsx(
          "bg-white border-2 border-black p-2 transition-all ring-purple-500",
          "placeholder:font-bold placeholder:text-gray-400 text-2xl",
          "outline-none w-full font-bold ring-0 focus-within:ring-4",
        )}
      >
        {children}
      </Slot>
    </label>
  );
}
