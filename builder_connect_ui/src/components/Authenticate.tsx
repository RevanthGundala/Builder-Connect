import React from "react";
import {
  GoogleLoginButton,
  DiscordLoginButton,
} from "react-social-login-buttons";
import { NextRouter, useRouter } from "next/router";

enum AuthProvider {
  Google,
  Discord,
}

export default function Authenticate() {
  const router = useRouter();

  async function handle_auth(auth_provider: AuthProvider) {
    try {
      console.log("Signing in...");
      let url = `${process.env.NEXT_PUBLIC_BASE_URL}/login?client_type=`;
      if (auth_provider === AuthProvider.Discord) {
        url += "discord";
      } else if (auth_provider === AuthProvider.Google) {
        url += "google";
      }
      const response = await fetch(url, { credentials: "include" });
      const login_url = await response.json();
      login_url === "/" ? router.push("/") : router.push(login_url);
    } catch (e) {
      console.log(e);
    }
  }
  return (
    <div className="flex flex-col space-y-2">
      <DiscordLoginButton onClick={() => handle_auth(AuthProvider.Discord)} />
      <GoogleLoginButton onClick={() => handle_auth(AuthProvider.Google)} />
    </div>
  );
}
