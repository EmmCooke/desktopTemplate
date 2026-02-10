import { useGreet } from "@/features/greet/hooks/use-greet";
import { useState } from "react";

export function Home() {
  const [name, setName] = useState("");
  const greet = useGreet();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (name.trim()) {
      greet.mutate(name);
    }
  };

  return (
    <div className="mx-auto max-w-md space-y-6 pt-8">
      <h2 className="text-2xl font-bold">Welcome</h2>
      <p className="text-gray-600">
        This is a Tauri 2 + React + Rust + SQLite template. Edit this page to get started.
      </p>

      <form onSubmit={handleSubmit} className="flex gap-2">
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="Enter a name..."
          className="flex-1 rounded border px-3 py-2 text-sm"
          aria-label="Name"
        />
        <button
          type="submit"
          className="rounded bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
        >
          Greet
        </button>
      </form>

      {greet.data && (
        <p role="status" className="rounded bg-gray-100 p-3 text-sm">
          {greet.data}
        </p>
      )}
    </div>
  );
}
