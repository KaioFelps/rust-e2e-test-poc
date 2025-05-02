import type { AppPageProps } from "@/types/libs/inertia";
import { usePage } from "@inertiajs/react";

export function useTypedPage<
  PageProps extends Record<never, never> = Record<never, never>,
  FlashProps extends Record<never, never> = Record<never, never>,
>() {
  return usePage<AppPageProps<PageProps, FlashProps>>();
}
