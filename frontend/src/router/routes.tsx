import { HomePage, AuthPage } from "./index";

export const routes = [
  {
    path: "/",
    element: <HomePage />,
  },
  {
    path: "/auth",
    element: <AuthPage />,
  },
];
