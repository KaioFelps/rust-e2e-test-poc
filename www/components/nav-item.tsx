import { Link, usePage } from "@inertiajs/react";
import clsx from "clsx";
import { type PropsWithChildren, useEffect, useState } from "react";

type NavItemProps = {
  href: string;
};

export function NavItem({ href, children }: PropsWithChildren<NavItemProps>) {
  const pageURI = usePage().url;

  const [isActive, setIsActive] = useState(false);

  useEffect(() => {
    const pageSegment = new URL(pageURI, window.location.href).pathname;
    setIsActive(pageSegment === href);
  }, [pageURI, href]);

  return (
    <Link
      className={clsx(
        "text-center font-black text-2xl text-white px-12 py-6",
        "text-shadow-black text-shadow-[0_2px_0]",
        "transition-all hover:bg-purple-400",
        isActive && "bg-amber-600",
      )}
      href={href}
    >
      {children}
    </Link>
  );
}
