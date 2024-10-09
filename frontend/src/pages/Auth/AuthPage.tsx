import React, { useState, useMemo, FormEvent, useTransition } from "react";
import Loader from "../../components/Loader";
import handleError from "../../helper/handleError";
import { useNavigate } from "react-router-dom";

export default function AuthPage() {
  const navigate = useNavigate();
  const [isPending, startTransition] = useTransition();

  const [authAction, setAuthAction] = useState<"login" | "register">("login");
  const [email, setEmail] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [fullName, setFullName] = useState<string>("");

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [needsEmail, setNeedsEmail] = useState<boolean>(false);
  const [needsPassword, setNeedsPassword] = useState<boolean>(false);
  const [needsFullName, setNeedsFullName] = useState<boolean>(false);

  const stars = useMemo(() => {
    return Array.from({ length: 200 }, (_, index) => <Star key={index} />);
  }, []);

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    let valid = true;
    if (!email) {
      setNeedsEmail(true);
      valid = false;
    }
    if (!email.includes("@") && email) {
      setNeedsEmail(true);
      valid = false;
      handleError("Correo no valido");
    }
    if (!password) {
      setNeedsPassword(true);
      valid = false;
    }
    if (!fullName) setNeedsFullName(true);

    if (authAction == "register" && !fullName) {
      valid = false;
    }
    if (!valid) {
      return;
    }
    auth();
  };
  async function auth() {
    setIsLoading(true);
    try {
      //if (authAction === "login") login();else register();
      startTransition(() => {
        navigate("/");
      });
    } catch (e) {
      handleError(e);
    } finally {
      setIsLoading(false);
    }
  }

  return (
    <div className="text-zinc-400 flex justify-center max-sm:items-end sm:items-center bg-gradient-to-tr from-gray-800 to-zinc-950 w-screen h-screen relative overflow-hidden">
      <div className="bg-slate-800 w-full max-w-md p-8 rounded-xl z-10 transition-all duration-300 ease-in-out">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          fill="currentColor"
          className="w-32 h-32 text-white mx-auto -mt-24 rounded-full bg-slate-800"
        >
          <path
            fillRule="evenodd"
            d="M18.685 19.097A9.723 9.723 0 0 0 21.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 0 0 3.065 7.097A9.716 9.716 0 0 0 12 21.75a9.716 9.716 0 0 0 6.685-2.653Zm-12.54-1.285A7.486 7.486 0 0 1 12 15a7.486 7.486 0 0 1 5.855 2.812A8.224 8.224 0 0 1 12 20.25a8.224 8.224 0 0 1-5.855-2.438ZM15.75 9a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z"
            clipRule="evenodd"
          />
        </svg>

        <div className="text-center">
          <h1 className="text-white -mt-4 mb-4 font-mono text-5xl font-bold">
            Electroshop
          </h1>
        </div>

        <div className="w-full mt-2 mb-4">
          <button
            onClick={() => setAuthAction("login")}
            className={`w-1/2 p-1 relative overflow-hidden duration-300 ${
              authAction === "login" ? "text-white" : "hover:text-white"
            }`}
          >
            Iniciar Sesión
            <span
              className={`absolute bottom-0 left-0 w-full h-0.5 rounded-full bg-white transform origin-right transition-transform duration-300 ${
                authAction === "login" ? "scale-x-100" : "scale-x-0"
              }`}
            ></span>
          </button>
          <button
            onClick={() => setAuthAction("register")}
            className={`w-1/2 p-1 relative overflow-hidden duration-300 ${
              authAction === "register" ? "text-white" : "hover:text-white"
            }`}
          >
            Registrarse
            <span
              className={`absolute bottom-0 left-0 w-full h-0.5 rounded-full bg-white transform origin-left transition-transform duration-300 ${
                authAction === "register" ? "scale-x-100" : "scale-x-0"
              }`}
            ></span>
          </button>
        </div>

        <form onSubmit={handleSubmit} className="flex flex-col justify-center">
          <div
            className={`transition-all duration-300 ease-in-out overflow-hidden ${
              authAction === "register"
                ? "max-h-20 opacity-100 mb-6"
                : "max-h-0 opacity-0 "
            }`}
          >
            <label
              htmlFor="fullName"
              className={`block mb-1 ${
                needsFullName && !fullName ? "text-red-500" : ""
              }`}
            >
              Nombre completo
            </label>
            <input
              id="fullName"
              className={`w-full text-white rounded-md border p-1.5 outline-none bg-slate-600 ${
                needsFullName && !fullName
                  ? " border-red-600"
                  : "border-transparent"
              }`}
              type="text"
              value={fullName}
              onChange={(e) => {
                setFullName(e.target.value);
                setNeedsFullName(false);
              }}
            />
          </div>

          <div className="mb-2">
            <label
              htmlFor="email"
              className={`block mb-1 ${needsEmail ? "text-red-500" : ""}`}
            >
              Correo
            </label>
            <input
              id="email"
              className={`w-full text-white rounded-md border p-1.5 outline-none bg-slate-600 ${
                needsEmail ? " border-red-600" : "border-transparent"
              }`}
              type="text"
              value={email}
              onChange={(e) => {
                setEmail(e.target.value);
                setNeedsEmail(false);
              }}
            />
          </div>

          <div className="my-4">
            <label
              htmlFor="password"
              className={`block mb-1 ${
                needsPassword && !password ? "text-red-500" : ""
              }`}
            >
              Contraseña
            </label>
            <input
              id="password"
              className={`w-full text-white rounded-md border p-1.5 outline-none bg-slate-600 ${
                needsPassword && !password
                  ? " border-red-600"
                  : "border-transparent"
              }`}
              type="password"
              value={password}
              onChange={(e) => {
                setPassword(e.target.value);
                setNeedsPassword(false);
              }}
            />
          </div>
          {isLoading || isPending ? (
            <div className="w-10 h-10 mx-auto mt-4">
              <Loader />
            </div>
          ) : (
            <input
              className="bg-zinc-950 text-white hover:text-zinc-400 hover:scale-95 duration-150 ease-in-out w-40 mx-auto cursor-pointer rounded-xl py-2 mt-4"
              type="submit"
              value="Continuar"
            />
          )}
        </form>
      </div>
      {stars}
    </div>
  );
}

const Star = React.memo(() => {
  const size = Math.random() * 2;
  const style = {
    top: `${Math.random() * 100}%`,
    left: `${Math.random() * 100}%`,
    width: `${size}px`,
    height: `${size}px`,
    animationDelay: `${Math.random() * 5}s`,
  };

  return (
    <div
      className="absolute z-0 bg-white rounded-full twinkleAnimation"
      style={style}
    />
  );
});
