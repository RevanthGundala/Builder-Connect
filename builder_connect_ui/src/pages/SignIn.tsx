import Authenticate from "@/components/Authenticate";
import ParticleBackground from "@/components/ParticleBackground";
import React from "react";

export default function SignIn() {
  return (
    <>
      <ParticleBackground />
      <div className="pt-24 bg-cover bg-center relative mx-auto">
        <div className="bg-white p-8 rounded shadow-md w-96 container mx-auto relative z-10 text-center">
          <h1 className="text-black text-center text-2xl mb-4">
            Builder Connect
          </h1>
          <form>
            <div className="mb-4">
              <label
                htmlFor="username"
                className="block text-gray-600 text-start"
              >
                Username
              </label>
              <input
                type="text"
                id="username"
                name="username"
                className="w-full px-3 py-2 border rounded focus:outline-none focus:border-black"
              />
            </div>
            <div className="mb-4">
              <label
                htmlFor="password"
                className="block text-gray-600 text-start"
              >
                Password
              </label>
              <input
                type="password"
                id="password"
                name="password"
                className="w-full px-3 py-2 border rounded focus:outline-none focus:border-black"
              />
            </div>
            <button
              type="submit"
              className="w-full bg-black hover:opacity-70 text-white rounded py-2"
            >
              Log In
            </button>
          </form>
          <div className="p-4 flex flex-row justify-center">
            <Authenticate />
          </div>
          {/* <p className="text-sm text-gray-500 mt-4">
            Don't have an account?{" "}
            <a href="#" className="text-black">
              Sign up
            </a>
          </p> */}
        </div>
      </div>
    </>
  );
}
