import { useState } from "react";
import { Link, Outlet, useLocation } from "react-router-dom";
import { Leaf, Plus } from "lucide-react";

const navigation: {
  name: string;
  href: string;
  icon?: React.ComponentType<{ className?: string }>;
}[] = [
  { name: "Plants", href: "/", icon: Leaf },
  { name: "Add hub", href: "/hub-add", icon: Plus },
];

function classNames(...classes: (string | false | undefined)[]) {
  return classes.filter(Boolean).join(" ");
}

function isActive(pathname: string, href: string) {
  return href === "/" ? pathname === "/" : pathname.startsWith(href);
}

export default function Shell() {
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const location = useLocation();

  return (
    <div className="min-h-screen bg-green-50">
      {sidebarOpen && (
        <div className="fixed inset-0 z-40 flex lg:hidden">
          <div className="fixed inset-0 bg-black/40" onClick={() => setSidebarOpen(false)} />
          <aside className="relative z-50 flex w-64 flex-col bg-white text-green-900 shadow-lg">
            <div className="flex h-16 items-center justify-between border-b border-green-100 px-4">
              <img src="/logo.jpeg" alt="Mycelium" className="h-10 w-auto" />
              <button onClick={() => setSidebarOpen(false)}>
                <svg viewBox="0 0 24 24" className="h-6 w-6" fill="none" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>

            <nav className="flex-1 space-y-1 px-3 py-4">
              {navigation.map((item) => {
                const active = isActive(location.pathname, item.href);
                return (
                  <Link
                    key={item.name}
                    to={item.href}
                    onClick={() => setSidebarOpen(false)}
                    className={classNames(
                      active ? "bg-green-100 font-semibold text-green-900" : "text-green-900 hover:bg-green-100",
                      "flex items-center gap-2 rounded-md px-3 py-2 text-sm",
                    )}
                  >
                    {item.icon && <item.icon className="h-5 w-5" />}
                    {item.name}
                  </Link>
                );
              })}
            </nav>
          </aside>
        </div>
      )}

      <aside className="hidden lg:fixed lg:inset-y-0 lg:z-30 lg:flex lg:w-72 lg:flex-col bg-white text-green-900 shadow">
        <div className="flex items-center border-b border-green-100">
          <img src="/logo.jpeg" alt="Mycelium" className="mx-auto h-32 w-auto p-2" />
        </div>

        <nav className="flex-1 space-y-1 px-4 py-6">
          {navigation.map((item) => {
            const active = isActive(location.pathname, item.href);
            return (
              <Link
                key={item.name}
                to={item.href}
                className={classNames(
                  active ? "bg-green-100 font-semibold text-green-900" : "text-green-900 hover:bg-green-100",
                  "flex items-center gap-2 rounded-md px-3 py-2 text-sm",
                )}
              >
                {item.icon && <item.icon className="h-5 w-5" />}
                {item.name}
              </Link>
            );
          })}
        </nav>
      </aside>

      <header className="sticky top-0 z-20 flex h-16 items-center gap-x-4 bg-white px-4 shadow lg:hidden">
        <button onClick={() => setSidebarOpen(true)} className="text-green-900">
          <svg viewBox="0 0 24 24" className="h-6 w-6" fill="none" stroke="currentColor" strokeWidth={2}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M4 6h16M4 12h16M4 18h16" />
          </svg>
        </button>
        <span className="text-sm font-semibold text-green-900">Your plants</span>
      </header>

      <main className="min-h-screen bg-green-50 lg:pl-72">
        <div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
          <Outlet />
        </div>
      </main>
    </div>
  );
}
