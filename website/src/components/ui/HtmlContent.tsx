import type { FC } from "hono/jsx";
import { twMerge } from "tailwind-merge";

export type HtmlContentProps = {
	html: string;
};

export const HtmlContent: FC<HtmlContentProps> = ({ html }) => {
	return (
		<div
			class={twMerge([
				"overflow-hidden",
				"[&_img]:mx-auto",
				"[&_img]:block",
				"[&_img]:max-w-full",
				"[&_img]:h-auto",
				"[&_img]:object-contain",
				"[&_img]:w-auto",
				"[&_pre]:overflow-x-auto",
				"[&_pre]:max-w-full",
				"[&_pre]:whitespace-pre-wrap",
				"[&_pre]:break-all",
				"[&_pre_code]:block",
				"[&_pre_code]:w-full",
			])}
			// biome-ignore lint/security/noDangerouslySetInnerHtml: typst-docsで生成されたHTMLを表示する
			dangerouslySetInnerHTML={{ __html: html }}
		/>
	);
};
