import { Routes, Route } from "react-router-dom";
import { RootLayout } from "./components/layouts/root-layout";
import { Home } from "./app/routes/home";

export function App() {
  return (
    <Routes>
      <Route element={<RootLayout />}>
        <Route path="/" element={<Home />} />
      </Route>
    </Routes>
  );
}
