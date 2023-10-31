import React from "react";
import Navbar from "../components/Navbar";
import { useEffect } from "react";
import { NextRouter, useRouter } from "next/router";

const LandingPage = () => {
  const router = useRouter();

  useEffect(() => {
    check_for_callback(router);

    async function check_for_callback(router: NextRouter) {
      try {
        const res = await fetch("http://localhost:8080/login/callback");
        const data = await res.json();
        router.push(`/profile/${data.sub_id}`);
      } catch (e) {
        console.log(e);
      }
    }
  });

  return (
    <div className="bg-gray-100">
      <Navbar />

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
          <p className="text-2xl text-white mt-4">Find Someone To Build With</p>
          <a
            href="#"
            className="mt-6 inline-block bg-blue-500 text-white font-semibold rounded-full py-3 px-8 hover:bg-blue-700"
          >
            Get Started
          </a>
        </div>
      </header>

      {/* Features Section */}
      <section className="py-16">
        <div className="container mx-auto text-center">
          <h2 className="text-3xl font-semibold">Key Features</h2>
          <div className="flex flex-wrap mt-12">
            {/* Feature 1 */}
            <div className="w-full md:w-1/2 lg:w-1/3 p-4">
              <div className="bg-white p-8 rounded shadow-lg">
                <h3 className="text-xl font-semibold">Feature 1</h3>
                <p className="mt-4">Description of feature 1.</p>
              </div>
            </div>
            {/* Repeat for more features */}
            <div className="w-full md:w-1/2 lg:w-1/3 p-4">
              <div className="bg-white p-8 rounded shadow-lg">
                <h3 className="text-xl font-semibold">Feature 1</h3>
                <p className="mt-4">Description of feature 1.</p>
              </div>
            </div>
            <div className="w-full md:w-1/2 lg:w-1/3 p-4">
              <div className="bg-white p-8 rounded shadow-lg">
                <h3 className="text-xl font-semibold">Feature 1</h3>
                <p className="mt-4">Description of feature 1.</p>
              </div>
            </div>
          </div>
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
      <footer className="bg-black py-8 text-center">
        <div className="container mx-auto">
          <p className="text-blue-500">Let us know what we can do better!</p>
          <p className="text-blue-500">BuilderConnect@gmail.com </p>
        </div>
      </footer>
    </div>
  );
};

export default LandingPage;
