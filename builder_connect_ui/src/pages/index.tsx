import React from "react";
import { useEffect, useState } from "react";
import { NextRouter, useRouter } from "next/router";
import Email from "@/components/Email";
import Link from "next/link";
import { useLocalStorage } from "usehooks-ts";
import ParticleBackground from "@/components/ParticleBackground";
import dynamic from "next/dynamic";
import Faq from "@/components/Faq/Faq";

const LandingPage = () => {
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");
  const Navbar = dynamic(() => import("../components/Navbar"), {
    ssr: false,
  });

  return (
    <>
      <ParticleBackground />
      <Navbar sub_id={sub_id} set_sub_id={set_sub_id} />
      <div className="bg-cover bg-center relative">
        <header className="pt-24 mx-auto">
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
        <Email />
        <section className="mt-24 mx-auto pb-10">
          <div className="container mx-auto relative z-10 text-center">
            <div className="w-full md:w-1/2 lg:w-1/3 p-4">
              <div className="bg-white p-8 rounded shadow-sm shadow-white">
                <h3 className="text-xl text-black">Improve Your Resume</h3>
              </div>
            </div>
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
        </section>
        <Faq />
      </div>
    </>
  );
};

export default LandingPage;
