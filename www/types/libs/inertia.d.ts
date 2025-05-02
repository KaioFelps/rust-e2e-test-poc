import type { PageProps as InertiaPageProps } from "@inertiajs/core";

export type AppPageProps<
  T extends Record<string, unknown> = Record<string, unknown>,
  Flash extends Record<string, string>,
> = {
  flash: Flash;
} & T;

declare module "@inertiajs/core/types" {
  export interface PageProps extends InertiaPageProps, AppPageProps {}
}
