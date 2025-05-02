import type { PropsWithChildren } from "react";

export function Main({ children }: PropsWithChildren) {
  return (
    <main className="my-20 mx-auto w-[calc(100%_-_48px)] max-w-7xl">
      {children}
    </main>
  );
}
