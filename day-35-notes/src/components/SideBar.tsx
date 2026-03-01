import { Link } from "react-router-dom";
import { Pencil2Icon } from "@radix-ui/react-icons";

export default function SideBar() {
  return (
    <div className="w-full max-w-[300px] xl:max-w-[350px] mr-6">
      <nav className="fixed left-4 top-[20px] max-w-[300px] xl:max-w-[350px] w-full bg-sumi-ink-0 text-white h-[calc(100vh-40px)] overflow-y-auto rounded-lg [&::-webkit-scrollbar]:hidden">
        <button className="flex items-center gap-3 mb-4 py-4 px-6 w-full text-dragon-white hover:bg-winter-blue hover:text-spring-blue border-b border-sumi-ink-3">
          <Pencil2Icon />
          <span>New Note</span>
        </button>

        <ul className="flex flex-col text-sm">
          {Array.from({ length: 10 }, (_, i) => (
            <li key={i}>
              <Link
                to="#"
                className="block w-full px-6 py-1 hover:bg-winter-blue hover:text-spring-blue"
              >
                Untitled
              </Link>
            </li>
          ))}
        </ul>
      </nav>
    </div>
  );
}
