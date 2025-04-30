import { Link } from "@inertiajs/react";
import clsx from "clsx";
import { PropsWithChildren } from "react";

type NavItemProps = {
    href: string;
}

export function NavItem({ href, children }: PropsWithChildren<NavItemProps>) {
    return (
        <Link
            className={clsx(
                "font-black text-2xl text-white text-shadow-black text-shadow-[0_2px_0] p-6",
                "transition-all hover:bg-purple-400"
            )}
            href={href}
        >
            {children}
        </Link>
    );
}