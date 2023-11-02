import React from "react";
import GoogleButton from "react-google-button";
import { NextRouter, useRouter } from "next/router";

export default function GoogleSignIn() {
  const router = useRouter();
  async function handle_google_sign_in(router: NextRouter) {
    console.log("Signing in with Google...");
    const url = "http://localhost:8080/login";
    const response = await fetch(url, { credentials: "include" });
    const login_url = await response.json();
    login_url === "/" ? router.push("/") : router.push(login_url);
  }
  return (
    <div>
      <GoogleButton onClick={() => handle_google_sign_in(router)} />
    </div>
  );
}
