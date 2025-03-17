import { Link } from "@heroui/link";
import { button as buttonStyles } from "@heroui/theme";

interface StartedButtonProps extends React.ComponentProps<"a"> {
  text: string;
}
export const StartedButton = ({ text, ...props }: StartedButtonProps) => {
  return <Link
  // isExternal
  className={`${buttonStyles({
    // color: "primary",
    radius: "full",
    variant: "shadow",
  })} group bg-pink-blue-animated h-11 text-base/4 items-center  text-pink-blue-foreground `}
  href={props.href}
>
  {text}
  <svg aria-hidden="true" fill="none" focusable="false" height="1em" role="presentation" viewBox="0 0 24 24" width="1em" className="size-4 group-hover:translate-x-0.5 outline-none transition-transform " ><path d="M16.835 6.91821L23.9166 13.9999L16.835 21.0815" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeMiterlimit="10" strokeWidth="2"></path><path d="M4.08325 14H23.7183" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeMiterlimit="10" strokeWidth="2"></path>
  </svg>
</Link>
}