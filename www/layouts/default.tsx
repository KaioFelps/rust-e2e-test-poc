import { NavItem } from "@/components/nav-item";
import { PropsWithChildren } from "react";


export function DefaultLayout({ children }: PropsWithChildren) {
  return (
    <div>
      <header className="bg-amber-400 border-b-2">
        <nav className="flex justify-center divide-x-2 divide-black">
          <NavItem href="/">Home</NavItem>
          <NavItem href="/new">Add New To-do</NavItem>
        </nav>
      </header>

      <main className="my-20 mx-auto w-[calc(100%_-_48px)] max-w-7xl">
        {children}
      </main>
    </div>
  );
}
