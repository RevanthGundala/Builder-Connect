import React from "react";
import { useEffect, useState } from "react";
import Email from "@/components/Email";
import Link from "next/link";
import ParticleBackground from "@/components/ParticleBackground";
import Faq from "@/components/Faq/Faq";
import Footer from "@/components/Footer";
import { TypeAnimation } from "react-type-animation";
import { Fade } from "react-awesome-reveal";
import useReadSession from "@/libs/useReadSession";

const LandingPage = () => {
  const [navigation, set_navigation] = useState({
    href: "/SignIn",
    text: "Get Started",
  });
  const { sub_id } = useReadSession();

  useEffect(() => {
    sub_id === "" || sub_id === undefined
      ? set_navigation({ href: "SignIn", text: "Get Started" })
      : set_navigation({ href: "profile/View", text: `Welcome Back` }); //TODO: replace with name
  }, [sub_id]);

  return (
    <>
      <ParticleBackground />
      <div className="bg-cover bg-center relative">
        <Fade>
          <header className="flex flex-col mb-96 mt-40 text-center">
            <div className="flex justify-center">
              {/* <img
                src="/images/hacker.png"
                alt="Hacker"
                className="w-72 h-72"
              /> */}
            </div>
            <h1 className="text-7xl text-white pt-20">The Buildwork</h1>
            <TypeAnimation
              className="text-2xl text-gray-400 mt-4"
              sequence={[
                "A platform connecting the next generation of builders.",
              ]}
              speed={50}
              wrapper="div"
              repeat={0}
            />
            <div className="flex justify-center">
              <Link
                href={navigation.href}
                className="mt-6 inline-block bg-white text-black rounded-full py-3 px-8 w-fit hover:opacity-60"
                shallow={true}
              >
                {navigation.text}
              </Link>
            </div>
          </header>
        </Fade>

        <div className="flex flex-col items-center text-white">
          <div className="flex justify-between items-center w-[64rem] h-64 my-96">
            <Fade>
              <section className="flex flex-col space-y-12 w-[20rem]">
                <h2 className="text-5xl">Improve Your Resume</h2>
                <p className="text-md font-medium text-gray-300">
                  Create a side project to make yourself stand out to employers
                </p>
              </section>
              <img
                src="/images/resume.jpeg"
                alt="Resume"
                className="w-96 h-96"
              />
            </Fade>
          </div>
          <div className="flex justify-between items-center w-[64rem] h-64 my-96">
            <Fade>
              <section className="flex flex-col space-y-12 w-[20rem]">
                <h2 className="text-5xl">Create a Startup</h2>
                <p className="text-md font-medium text-gray-300">
                  Take the future into your own hands and create the next big
                  thing
                </p>
              </section>
              <img
                src="/images/startup.jpeg"
                alt="Resume"
                className="w-96 h-96"
              />
            </Fade>
          </div>
          <div className="flex justify-between items-center w-[64rem] h-64 my-96">
            <Fade>
              <section className="flex flex-col space-y-12 w-[20rem]">
                <h2 className="text-5xl">Meet Others Like You</h2>
                <p className="text-md font-medium text-gray-300">
                  Find others who share your interests and ambitions
                </p>
              </section>
              <img src="/images/meet.jpeg" alt="Resume" className="w-96 h-96" />
            </Fade>
          </div>
        </div>
        <Fade>
          <Faq />
          <Email />
        </Fade>
        <Footer />
      </div>
    </>
  );
};

export default LandingPage;
