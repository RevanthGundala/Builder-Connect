import FaqSection from "./FaqSection";

export default function Faq() {
  return (
    <div className="flex flex-col items-center text-white mb-60">
      <h1 className="text-4xl my-20">Frequently Asked Questions</h1>
      <div className="flex flex-col">
        <FaqSection
          outer_text="What is The Buildwork?"
          inner_text="The Buildwork is a platform dedicated to connecting the next generation of builders. We aim to help ambitious people find others with similar interests to build projects together."
        />
        <FaqSection
          outer_text="Who is The Buildwork For?"
          inner_text="If you want to work on a side project or have an amazing idea for a startup and don't have the right person to work with, then
          The Buildwork is for you!"
        />
        <FaqSection
          outer_text="How does it work?"
          inner_text="The Buildwork uses a matching algorithm to connect people who have
        similar interests. Fill out your profile, and find someone to build with!"
        />
        <FaqSection
          outer_text="How do I get started?"
          inner_text={
            "1. Sign Up (Discord recommended, but Google is also supported)\n\n2. Fill out your profile\n\n3. Start swiping!\n\n4. When you get a match, you can chat with the other person to see if you have a project in mind that you both want to work on."
          }
        />
      </div>
    </div>
  );
}
