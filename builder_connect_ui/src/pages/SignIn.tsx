import Authenticate from "@/components/Authenticate";
import React from "react";
import GoogleButton from "react-google-button";

export default function SignIn() {
  return (
    <div className="bg-gray-100 h-screen flex items-center justify-center">
      <div className="bg-white p-8 rounded shadow-md w-96">
        <h1 className="text-blue-500 text-center text-2xl font-semibold mb-4">
          Builder Connect
        </h1>
        <form>
          <div className="mb-4">
            <label htmlFor="username" className="block text-gray-600">
              Username
            </label>
            <input
              type="text"
              id="username"
              name="username"
              className="w-full px-3 py-2 border rounded focus:outline-none focus:border-blue-400"
            />
          </div>
          <div className="mb-4">
            <label htmlFor="password" className="block text-gray-600">
              Password
            </label>
            <input
              type="password"
              id="password"
              name="password"
              className="w-full px-3 py-2 border rounded focus:outline-none focus:border-blue-400"
            />
          </div>
          <button
            type="submit"
            className="w-full bg-blue-500 text-white rounded py-2"
          >
            Log In
          </button>
        </form>
        <div className="p-4 flex flex-row justify-center">
          <Authenticate />
        </div>
        <p className="text-sm text-gray-500 mt-4">
          Don't have an account?{" "}
          <a href="#" className="text-blue-500">
            Sign up
          </a>
        </p>
      </div>
    </div>
  );
}
