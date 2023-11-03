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
        className="py-8 bg-cover bg-center relative"
        style={{ backgroundImage: 'url("your-hero-image.jpg")' }}
      >
        <div className="absolute inset-0 bg-black opacity-50"></div>
        <div className="container mx-auto relative z-10 text-center">
          <h1 className="text-4xl text-white font-extrabold leading-tight">
            About
          </h1>
        </div>
      </header>

      {/* Features Section */}
      <section className="py-16 bg-gray-100">
        <div className="container mx-auto text-center">
          <h2 className="text-2xl font-bold text-blue-500 p-2">
            What is Builder Connect?
          </h2>
          <p className="text-gray-800 text-lg text-center">
            Builder Connect is a platform that connects builders with one
            another.
          </p>
          <br />
          <h2 className="text-2xl font-bold text-blue-500 p-2">
            Who is Builder Connect For?
          </h2>
          <p className="text-gray-800 text-lg text-center">
            If you want to work on a side project or have an amazing idea for a
            startup <br /> and don't have the right person to work with, then
            Builder Connect is for you!
          </p>
          <br />
          <h2 className="text-2xl font-bold text-blue-500 p-2">
            How does it work?
          </h2>
          <p className="text-gray-800 text-lg text-center">
            If you have ever used Tinder or a similar matching app, then you
            will be pretty comfortable. <br />
            Builder Connect uses a matching algorithm to connect people who have
            similar interests.
          </p>
          <br />
          <h2 className="text-2xl font-bold text-blue-500 p-2">
            How do I get started?
          </h2>
          <p className="text-gray-800 text-lg">
            1. Sign up for an account (Currently only Google Sign In is
            supported)
            <br />
            2. Fill out your profile
            <br />
            3. Start swiping!
            <br />
            4. When you get a match, you can have a quick chat with the other
            person <br />
            to see if you have a project in mind that you both want to work on.
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
