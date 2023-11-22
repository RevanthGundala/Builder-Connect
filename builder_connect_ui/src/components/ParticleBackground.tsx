import React from "react";
import { useCallback } from "react";
import Particles from "react-particles";
import type { Container, Engine } from "tsparticles-engine";
import { loadFull } from "tsparticles"; // if you are going to use `loadFull`, install the "tsparticles" package too.
import { particles_config } from "@/config/particles-config";

export default function ParticleBackground() {
  const particlesInit = useCallback(async (engine: Engine) => {
    await loadFull(engine);
  }, []);

  return (
    <div id="particle-background">
      <Particles
        id="tsparticles"
        init={particlesInit}
        options={particles_config}
        height="100vh"
        width="100vw"
      />
    </div>
  );
}
