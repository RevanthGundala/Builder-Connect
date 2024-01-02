import { PlusIcon, MinusIcon } from "@heroicons/react/24/solid";
import { Fade } from "react-awesome-reveal";

export default function FaqSection({
  outer_text,
  inner_text,
}: {
  outer_text: string;
  inner_text: string;
}) {
  return (
    <details className="group w-[32rem]">
      <summary className="flex cursor-pointer items-center py-4 transition duration-100 ease-in-out">
        <div className="flex flex-1 text-2xl font-medium text-white p-2">
          {outer_text}
        </div>
        <div className="flex">
          <PlusIcon className="h-4 w-4 group-open:hidden" />
          <MinusIcon className="hidden h-4 w-4 group-open:block" />
        </div>
      </summary>
      <Fade>
        <p className="text-gray-300 p-2 whitespace-pre-wrap">{inner_text}</p>
      </Fade>
    </details>
  );
}
