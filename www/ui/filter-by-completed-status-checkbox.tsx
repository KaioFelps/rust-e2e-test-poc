import { Check } from "@phosphor-icons/react/dist/ssr/Check";
import { X } from "@phosphor-icons/react/dist/ssr/X";
import * as Checkbox from "@radix-ui/react-checkbox";
import clsx from "clsx";

export type CompletedState = Checkbox.CheckedState;

type FilterByCompletedStatusCheckboxProps = {
    setState: React.Dispatch<React.SetStateAction<Checkbox.CheckedState>>,
    state: Checkbox.CheckedState;
};

export function FilterByCompletedStatusCheckbox({
    setState,
    state,
}: FilterByCompletedStatusCheckboxProps) {
    return (
        <Checkbox.Root
            checked={state}
            onCheckedChange={(_) => setState(prev => handleNextState(prev))}
            className={clsx(
                "size-16 border-2 border-black transition-all duration-75",
                "outline-none ring-0 focus:ring-4",
                "hover:scale-105",
                "active:scale-95",
                state === "indeterminate" && "bg-gray-200 ring-sky-300",
                state === true && "bg-amber-400 ring-yellow-400",
                state === false && "bg-red-400 ring-red-300"
            )}
        >
            <div className="text-white size-full grid place-items-center">
                {state === false && <X size={32} weight="bold" />}
                {state === true && <Check size={32} weight="bold" />}
            </div>
        </Checkbox.Root>
    )
}

function handleNextState(current: CompletedState): CompletedState {
    switch (current) {
        case "indeterminate": return true;
        case true: return false;
        case false: return "indeterminate"
    }
}