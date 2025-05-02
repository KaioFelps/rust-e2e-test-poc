import { NavItem } from "@/components/nav-item";
import { TooltipProvider } from "@radix-ui/react-tooltip";
import type { PropsWithChildren } from "react";

export function DefaultLayout({ children }: PropsWithChildren) {
  return (
    <TooltipProvider delayDuration={50}>
      <header className="bg-amber-400 border-b-2">
        <nav className="flex justify-center divide-x-2 divide-black">
          <NavItem href="/">Home</NavItem>
          <NavItem href="/new">Add New To-do</NavItem>
        </nav>
      </header>

      <main className="my-20 mx-auto w-[calc(100%_-_48px)] max-w-7xl">
        {children}
      </main>
    </TooltipProvider>
  );
}
