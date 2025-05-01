import { PropsWithChildren, ReactNode, useCallback, useState } from "react";
import * as Tooltip from "@radix-ui/react-tooltip";
import clsx from "clsx";

type CheckboxTooltipProps = {
    message: ReactNode;
}

export function CheckboxTooltip({ children, message }: PropsWithChildren<CheckboxTooltipProps>) {
  return (
    <Tooltip.Root>
      <Tooltip.Trigger asChild>
        <div className="flex items-center">
          {children}
        </div>
      </Tooltip.Trigger>

      <Tooltip.Portal>
        <Tooltip.Content
          collisionPadding={24}
          sideOffset={12}
          alignOffset={12}
          className={clsx(
            "px-12 py-4 bg-gray-100 border-2 border-black shadow-[4px_4px_0] shadow-black"
          )}
        >
          {message}
        </Tooltip.Content>
      </Tooltip.Portal>
    </Tooltip.Root>
  )
}