/**
 * Utility to conditionally join class names.
 * Replace with clsx + tailwind-merge when adding shadcn/ui.
 */
export function cn(...classes: (string | boolean | undefined | null)[]) {
  return classes.filter(Boolean).join(" ");
}
