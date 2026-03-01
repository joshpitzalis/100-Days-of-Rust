import { useEffect, useRef } from "react";
import EasyMDE from "easymde";
import "easymde/dist/easymde.min.css";

interface EasyMdeProps {
  value: string;
  onChange: (value: string) => void;
}

export default function EasyMde({ value, onChange }: EasyMdeProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const editorRef = useRef<EasyMDE | null>(null);
  const initialValueSet = useRef(false);

  useEffect(() => {
    if (!textareaRef.current) return;

    const easymde = new EasyMDE({
      element: textareaRef.current,
      toolbar: false,
    });

    editorRef.current = easymde;

    easymde.codemirror.on("change", () => {
      onChange(easymde.value());
    });

    return () => {
      easymde.toTextArea();
    };
  }, []);

  // Set initial value once when it becomes available (mirrors watchOnce behavior)
  useEffect(() => {
    if (!initialValueSet.current && value && editorRef.current) {
      editorRef.current.value(value);
      initialValueSet.current = true;
    }
  }, [value]);

  return <textarea ref={textareaRef} />;
}
