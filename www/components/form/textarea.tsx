import type { ToString } from "@/types/interfaces/to-string";
import { InputBase, type InputBaseProps } from "./input-base";

type TextareaProps<T> = {
  name: string;
  setValue: (value: string) => void;
  defaultValue?: T;
  placeholder?: string;
} & InputBaseProps<T>;

export function Textarea<T extends ToString>({
  name,
  setValue,
  defaultValue,
  placeholder,
  ...props
}: TextareaProps<T>) {
  return (
    <InputBase<T> {...props}>
      <textarea
        rows={10}
        defaultValue={defaultValue?.toString()}
        name={name}
        placeholder={placeholder}
        onInput={(event) => setValue(event.currentTarget.value)}
      />
    </InputBase>
  );
}
