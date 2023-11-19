import React from "react";
import Navbar from "../components/Navbar";
import { useEffect, useState } from "react";
import { NextRouter, useRouter } from "next/router";
import Footer from "@/components/Footer";
import Link from "next/link";
import { useLocalStorage } from "usehooks-ts";

const LandingPage = () => {
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");

  return (
    <div className="bg-gray-100">
      <Navbar />
      {/* Hero Section */}
      <header className="py-16 bg-cover bg-center relative">
        <div className="absolute inset-0 bg-black opacity-50"></div>
        <div className="container mx-auto relative z-10 text-center">
          <h1 className="text-5xl text-white font-extrabold leading-tight">
            Builder Connect
          </h1>
          <p className="text-2xl text-white mt-4">Build the Future</p>
          {sub_id !== "" ? (
            <Link
              href={`/Swipe`}
              className="mt-6 inline-block bg-blue-500 text-white font-semibold rounded-full py-3 px-8 hover:bg-blue-700"
            >
              Get Started
            </Link>
          ) : (
            <Link
              href="/SignIn"
              className="mt-6 inline-block bg-blue-500 text-white font-semibold rounded-full py-3 px-8 hover:bg-blue-700"
            >
              Get Started
            </Link>
          )}
        </div>
      </header>

      {/* Features Section */}
      <section className="py-16">
        <div className="container mx-auto text-center">
          <h2 className="text-3xl text-blue-500 font-semibold">Features</h2>
          <div className="flex flex-wrap mt-12">
            {/* Feature 1 */}
            <div className="w-full md:w-1/2 lg:w-1/3 p-4">
              <div className="bg-white p-8 rounded shadow-lg">
                <h3 className="text-xl text-blue-500 font-semibold">
                  Improve Your Resume
                </h3>
              </div>
            </div>
            {/* Repeat for more features */}
            <div className="w-full md:w-1/2 lg:w-1/3 p-4">
              <div className="bg-white p-8 rounded shadow-lg">
                <h3 className="text-xl text-blue-500 font-semibold">
                  Create a Startup
                </h3>
              </div>
            </div>
            <div className="w-full md:w-1/2 lg:w-1/3 p-4">
              <div className="bg-white p-8 rounded shadow-lg">
                <h3 className="text-xl text-blue-500 font-semibold">
                  Meet Others Like You
                </h3>
              </div>
            </div>
          </div>
        </div>
      </section>
      <section className="bg-gray-100 mb-4">
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
            1. Sign up for an account (Discord recommended, but Google is also
            supported)
            <br />
            2. Fill out your profile
            <br />
            3. Start swiping!
            <br />
            4. When you get a match, you can chat with the other person <br />
            to see if you have a project in mind that you both want to work on.
          </p>
        </div>
      </section>
      <section className="bg-blue-500 text-white py-16">
        <div className="container mx-auto text-center">
          <h2 className="text-3xl font-semibold">Get Notified</h2>
          <p className="mt-4">Want to get an email when you get a match?</p>
          <a
            href="/"
            className="mt-6 inline-block bg-white text-blue-500 font-semibold rounded-full py-3 px-8 hover:bg-white hover:text-blue-500"
          >
            Coming Soon!
          </a>
        </div>
      </section>

      {/* Footer */}
      <Footer />
    </div>
  );
};

export default LandingPage;
