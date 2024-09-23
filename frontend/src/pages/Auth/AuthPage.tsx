import React, { useState, useMemo, FormEvent } from "react";

export default function AuthPage() {
  const [authAction, setAuthAction] = useState<"login" | "register">("login");
  const [email, setEmail] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [fullName, setFullName] = useState<string>("");

  const [needsEmail, setNeedsEmail] = useState<boolean>(false);
  const [needsPassword, setNeedsPassword] = useState<boolean>(false);
  const [needsFullName, setNeedsFullName] = useState<boolean>(false);

  const stars = useMemo(() => {
    return Array.from({ length: 200 }, (_, index) => <Star key={index} />);
  }, []);

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!email) {
      setNeedsEmail(true);
    }
    if (!password) {
      setNeedsPassword(true);
    }
    if (!fullName) setNeedsFullName(true);

    if (authAction == "register" && !fullName) {
      return;
    }
    if (!email || !password) return;
    console.log(`You tried to ${authAction}`);
    console.log({ email, password, fullName });
  };

  return (
    <div className="text-zinc-400 flex justify-center max-sm:items-end sm:items-center bg-gradient-to-tr from-gray-800 to-zinc-950 w-screen h-screen relative overflow-hidden">
      <div className="bg-slate-800 w-full max-w-md p-8 rounded-xl z-10 transition-all duration-300 ease-in-out">
        <div className="w-32 h-32 bg-black mx-auto -mt-24 rounded-full"></div>
        <div className="w-full mt-2 mb-6">
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
                ? "max-h-20 opacity-100 mb-8"
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
              onChange={(e) => setFullName(e.target.value)}
            />
          </div>

          <div className="mb-4">
            <label
              htmlFor="email"
              className={`block mb-1 ${
                needsEmail && !email ? "text-red-500" : ""
              }`}
            >
              Correo
            </label>
            <input
              id="email"
              className={`w-full text-white rounded-md border p-1.5 outline-none bg-slate-600 ${
                needsEmail && !email ? " border-red-600" : "border-transparent"
              }`}
              type="text"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
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
              type="text"
              value={password}
              onChange={(e) => {
                setPassword(e.target.value);
                setNeedsPassword(false);
              }}
            />
          </div>

          <input
            className="bg-zinc-950 text-white hover:text-zinc-400 hover:scale-95 duration-150 ease-in-out w-40 mx-auto cursor-pointer rounded-xl py-2 mt-4"
            type="submit"
            value={authAction === "login" ? "Iniciar Sesión" : "Registrarse"}
          />
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
