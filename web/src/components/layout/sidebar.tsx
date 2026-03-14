"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn } from "@/lib/utils";

const navigation = [
  { name: "Dashboard", href: "/", icon: "◉" },
  { name: "Containers", href: "/containers", icon: "▦" },
  { name: "Pods", href: "/pods", icon: "⬡" },
  { name: "Images", href: "/images", icon: "◧" },
  { name: "Volumes", href: "/volumes", icon: "▥" },
  { name: "Networks", href: "/networks", icon: "◈" },
  { name: "Events", href: "/events", icon: "◎" },
  { name: "Nodes", href: "/nodes", icon: "⬢" },
];

export function Sidebar() {
  const pathname = usePathname();

  return (
    <aside className="flex w-60 flex-col border-r border-[var(--border)] bg-[var(--card)]">
      <div className="flex h-14 items-center gap-2 border-b border-[var(--border)] px-4">
        <span className="text-lg font-bold tracking-tight text-[var(--accent)]">PodMesh</span>
        <span className="rounded bg-[var(--muted)] px-1.5 py-0.5 text-[10px] font-medium text-[var(--muted-foreground)]">
          v0.1
        </span>
      </div>

      <nav className="flex-1 space-y-1 p-3">
        {navigation.map((item) => {
          const active = pathname === item.href;
          return (
            <Link
              key={item.href}
              href={item.href}
              className={cn(
                "flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors",
                active
                  ? "bg-[var(--accent)]/10 text-[var(--accent)]"
                  : "text-[var(--muted-foreground)] hover:bg-[var(--muted)] hover:text-[var(--foreground)]"
              )}
            >
              <span className="text-base">{item.icon}</span>
              {item.name}
            </Link>
          );
        })}
      </nav>

      <div className="border-t border-[var(--border)] p-3">
        <div className="flex items-center gap-2 rounded-md px-3 py-2 text-xs text-[var(--muted-foreground)]">
          <span className="h-2 w-2 rounded-full bg-[var(--success)]" />
          Server connected
        </div>
      </div>
    </aside>
  );
}
