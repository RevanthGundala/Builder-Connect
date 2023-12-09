import React from "react";
import { useEffect, useState } from "react";
import { NextRouter, useRouter } from "next/router";
import Footer from "@/components/Footer";
import Link from "next/link";
import { useLocalStorage } from "usehooks-ts";
import ParticleBackground from "@/components/ParticleBackground";
import dynamic from "next/dynamic";

const LandingPage = () => {
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");
  const Navbar = dynamic(() => import("../components/Navbar"), {
    ssr: false,
  });

  return (
    <>
      <ParticleBackground />
      <Navbar sub_id={sub_id} set_sub_id={set_sub_id} />
      <header className="pt-24 bg-cover bg-center relative mx-auto">
        <div className="container mx-auto relative z-10 text-center">
          <h1 className="text-7xl text-white">Builder Connect</h1>
          <p className="text-2xl text-gray-400 mt-4">
            A platform dedicated to connecting the next generation of builders
          </p>
          <Link
            href={`/Swipe`}
            className="mt-6 inline-block bg-white text-black rounded-full py-3 px-8 hover:opacity-60"
            shallow={true}
          >
            Get Started
          </Link>
        </div>
      </header>

      {/* Features Section */}
      <section className="mt-24 bg-cover bg-center relative mx-auto border-b border-gray-200 pb-10">
        <div className="container mx-auto relative z-10 text-center">
          <div className="flex flex-wrap mt-2">
            {/* Feature 1 */}
            <div className="w-full md:w-1/2 lg:w-1/3 p-4">
              <div className="bg-white p-8 rounded shadow-sm shadow-white">
                <h3 className="text-xl text-black">Improve Your Resume</h3>
              </div>
            </div>
            {/* Repeat for more features */}
            <div className="w-full md:w-1/2 lg:w-1/3 p-4">
              <div className="bg-white p-8 rounded shadow-white shadow-sm">
                <h3 className="text-xl text-black">Create a Startup</h3>
              </div>
            </div>
            <div className="w-full md:w-1/2 lg:w-1/3 p-4">
              <div className="bg-white p-8 rounded shadow-sm shadow-white">
                <h3 className="text-xl text-black">Meet Others Like You</h3>
              </div>
            </div>
          </div>
        </div>
      </section>
      <section className="bg-cover bg-center relative mx-auto border-b border-gray-200 py-10">
        <div className="container mx-auto relative z-10 text-center">
          <h2 className="text-2xl font-bold text-white p-2">
            What is Builder Connect?
          </h2>
          <p className="text-gray-300 text-lg text-center">
            Builder Connect is a platform that connects builders with one
            another.
          </p>
          <br />
          <h2 className="text-2xl font-bold text-white p-2">
            Who is Builder Connect For?
          </h2>
          <p className="text-gray-300 text-lg text-center">
            If you want to work on a side project or have an amazing idea for a
            startup <br /> and don't have the right person to work with, then
            Builder Connect is for you!
          </p>
          <br />
          <h2 className="text-2xl font-bold text-white p-2">
            How does it work?
          </h2>
          <p className="text-gray-300 text-lg text-center">
            If you have ever used Tinder or a similar matching app, then you
            will be pretty comfortable. <br />
            Builder Connect uses a matching algorithm to connect people who have
            similar interests.
          </p>
          <br />
          <h2 className="text-2xl font-bold text-white p-2">
            How do I get started?
          </h2>
          <p className="text-gray-300 text-lg">
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
      {/* Footer */}
      <Footer />
    </>
  );
};

export default LandingPage;
