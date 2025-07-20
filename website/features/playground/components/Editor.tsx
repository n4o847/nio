"use client";

import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { type FC, useEffect, useRef, useState } from "react";

export interface EditorProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, "onChange"> {
  defaultValue?: string;
  theme?: "light" | "dark";
  onChange?: (value: string) => void;
}

export const Editor: FC<EditorProps> = ({
  defaultValue,
  theme,
  onChange,
  ...props
}) => {
  const [editor, setEditor] =
    useState<monaco.editor.IStandaloneCodeEditor | null>(null);
  const [value, _setValue] = useState(defaultValue || "");
  const monacoEl = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (monacoEl.current) {
      const editor = monaco.editor.create(monacoEl.current, {
        value,
        automaticLayout: true,
      });

      setEditor(editor);

      return () => {
        editor.dispose();
        setEditor(null);
      };
    }
  }, [value]);

  useEffect(() => {
    monaco.editor.setTheme(theme === "dark" ? "vs-dark" : "vs-light");
  }, [theme]);

  useEffect(() => {
    if (editor && onChange) {
      const disposable = editor.onDidChangeModelContent(() => {
        const value = editor.getValue();
        onChange(value);
      });

      return () => {
        disposable.dispose();
      };
    }
  }, [editor, onChange]);

  return <div ref={monacoEl} {...props}></div>;
};
