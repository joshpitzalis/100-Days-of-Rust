import { useState, useEffect, useRef } from "react";
import { ChevronDownIcon, Cross1Icon, CheckIcon } from "@radix-ui/react-icons";
import { getAllTags } from "../db";

interface TagsInputProps {
  value: string[];
  onChange: (tags: string[]) => void;
}

export default function TagsInput({ value, onChange }: TagsInputProps) {
  const [searchTerm, setSearchTerm] = useState("");
  const [options, setOptions] = useState<string[]>([]);
  const [open, setOpen] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState(-1);
  const containerRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    getAllTags().then(setOptions);
  }, []);

  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const filteredOptions = options.filter(
    (opt) =>
      opt.toLowerCase().includes(searchTerm.toLowerCase()) &&
      !value.includes(opt),
  );

  function addTag(tag: string) {
    if (!value.includes(tag)) {
      onChange([...value, tag]);
    }
    setSearchTerm("");
  }

  function removeTag(tag: string) {
    onChange(value.filter((t) => t !== tag));
  }

  function toggleTag(tag: string) {
    if (value.includes(tag)) {
      removeTag(tag);
    } else {
      addTag(tag);
    }
  }

  const canCreate = searchTerm.trim() && !options.includes(searchTerm.trim()) && !value.includes(searchTerm.trim());

  function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Enter") {
      e.preventDefault();
      if (highlightedIndex >= 0 && highlightedIndex < filteredOptions.length) {
        toggleTag(filteredOptions[highlightedIndex]);
      } else if (highlightedIndex === filteredOptions.length && canCreate) {
        addTag(searchTerm.trim());
      } else if (searchTerm.trim()) {
        addTag(searchTerm.trim());
      }
      setSearchTerm("");
    } else if (e.key === "Backspace" && searchTerm === "" && value.length > 0) {
      removeTag(value[value.length - 1]);
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      const max = filteredOptions.length + (canCreate ? 1 : 0);
      setHighlightedIndex((i) => (i < max - 1 ? i + 1 : 0));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      const max = filteredOptions.length + (canCreate ? 1 : 0);
      setHighlightedIndex((i) => (i > 0 ? i - 1 : max - 1));
    } else if (e.key === "Escape") {
      setOpen(false);
    }
  }

  return (
    <div ref={containerRef} className="my-4 mx-auto relative">
      <div className="inline-flex items-center justify-between rounded-lg p-2 text-sm leading-none gap-2 bg-sumi-ink-5 text-spring-blue shadow-md outline-none w-full">
        <div className="flex gap-2 items-center rounded-lg flex-wrap flex-1">
          {value.map((tag) => (
            <span
              key={tag}
              className="flex items-center justify-center gap-2 text-spring-blue bg-winter-blue rounded px-2 py-1"
            >
              <span className="text-sm">{tag}</span>
              <button
                onClick={(e) => {
                  e.preventDefault();
                  removeTag(tag);
                }}
              >
                <Cross1Icon />
              </button>
            </span>
          ))}
          <input
            ref={inputRef}
            value={searchTerm}
            onChange={(e) => {
              setSearchTerm(e.target.value);
              setOpen(true);
              setHighlightedIndex(-1);
            }}
            onFocus={() => setOpen(true)}
            onKeyDown={handleKeyDown}
            placeholder="Tags..."
            className="focus:outline-none flex-1 min-w-[120px] rounded !bg-transparent placeholder:text-fuji-gray px-1"
          />
        </div>
        <button onClick={() => setOpen(!open)}>
          <ChevronDownIcon />
        </button>
      </div>

      {open && (filteredOptions.length > 0 || searchTerm.trim()) && (
        <div className="absolute z-10 w-full mt-2 bg-sumi-ink-5 overflow-hidden rounded shadow-lg">
          <div className="p-2">
            <div className="px-6 text-xs leading-[25px] text-spring-blue">
              Tags
            </div>
            {filteredOptions.map((option, index) => (
              <div
                key={option}
                className={`text-sm leading-none text-spring-blue rounded-sm flex items-center h-[25px] pr-9 pl-6 relative select-none cursor-pointer ${
                  index === highlightedIndex
                    ? "bg-winter-blue text-dragon-white outline-none"
                    : ""
                }`}
                onMouseEnter={() => setHighlightedIndex(index)}
                onClick={() => toggleTag(option)}
              >
                {value.includes(option) && (
                  <span className="absolute left-0 w-6 inline-flex items-center justify-center">
                    <CheckIcon />
                  </span>
                )}
                <span>{option}</span>
              </div>
            ))}
            {canCreate && (
              <div
                className={`text-sm leading-none text-spring-blue rounded-sm flex items-center h-[25px] pr-9 pl-6 relative select-none cursor-pointer ${
                  highlightedIndex === filteredOptions.length
                    ? "bg-winter-blue text-dragon-white outline-none"
                    : ""
                }`}
                onMouseEnter={() => setHighlightedIndex(filteredOptions.length)}
                onClick={() => addTag(searchTerm.trim())}
              >
                <span>Create "{searchTerm.trim()}"</span>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
