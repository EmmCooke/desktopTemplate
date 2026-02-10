import { Outlet } from "react-router-dom";

export function RootLayout() {
  return (
    <div className="flex min-h-screen flex-col">
      <header className="flex h-12 items-center border-b px-4" data-tauri-drag-region>
        <h1 className="text-sm font-semibold">Desktop Template</h1>
      </header>
      <main className="flex-1 p-4">
        <Outlet />
      </main>
    </div>
  );
}
