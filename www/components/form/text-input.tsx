import { InputBase, type InputBaseProps } from "./input-base";
import type { ToString } from "@/types/interfaces/to-string";

type InputProps<T> = {
  name: string;
  setValue: (value: string) => void;
  defaultValue?: T;
  placeholder?: string;
} & InputBaseProps<T>;

export function TextInput<T extends ToString>({
  setValue,
  defaultValue,
  placeholder,
  name,
  ...props
}: InputProps<T>) {
  return (
    <InputBase<T> {...props}>
      <input
        type="text"
        onInput={(event) => setValue(event.currentTarget.value)}
        defaultValue={defaultValue?.toString()}
        placeholder={placeholder}
        name={name}
      />
    </InputBase>
  );
}
