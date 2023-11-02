import Footer from "@/components/Footer";
import Navbar from "@/components/Navbar";
import React, { useState, useEffect } from "react";

export default function About() {
  const [is_connected, set_is_connected] = useState(false);

  useEffect(() => {
    check_session();

    async function check_session() {
      try {
        const url = process.env.NEXT_PUBLIC_BASE_URL + `/get_session`;
        const res = await fetch(url, { credentials: "include" });
        const data = await res.json();
        data === "Not set." ? set_is_connected(false) : set_is_connected(true);
        is_connected ? console.log("Connected") : console.log("Not connected");
      } catch (err) {
        console.log(err);
      }
    }
  }, [is_connected]);
  return (
    <div className="bg-gray-100">
      <Navbar is_connected={is_connected} />

      {/* Hero Section */}
      <header
        className="py-16 bg-cover bg-center relative"
        style={{ backgroundImage: 'url("your-hero-image.jpg")' }}
      >
        <div className="absolute inset-0 bg-black opacity-50"></div>
        <div className="container mx-auto relative z-10 text-center">
          <h1 className="text-5xl text-white font-extrabold leading-tight">
            Builder Connect
          </h1>
        </div>
      </header>

      {/* Features Section */}
      <section className="py-16">
        <div className="container mx-auto text-center">
          <h2 className="text-3xl text-blue-500 font-semibold pb-2">About</h2>
          <p className="text-black">
            Builder Connect is a platform that connects builders with one
            another. It is designed for people who want to create projects to do
            multiple things
          </p>
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-blue-500 text-white py-16">
        <div className="container mx-auto text-center">
          <h2 className="text-3xl font-semibold">Get Notified</h2>
          <p className="mt-4">
            Want to get an email when you get a match? Sign up now!
          </p>
          <a
            href="#"
            className="mt-6 inline-block bg-white text-blue-500 font-semibold rounded-full py-3 px-8 hover:bg-white hover:text-blue-500"
          >
            Sign Up
          </a>
        </div>
      </section>

      {/* Footer */}
      <Footer />
    </div>
  );
}
