import type { IconDefinition } from '@fortawesome/free-solid-svg-icons';

export const themes = ['dark', 'light'] as const;
export type Theme = (typeof themes)[number];

export type NavLink = {
  target: {
    href: string;
    newTab?: boolean;
  };
  content: {
    text?: string;
    icon?: IconDefinition;
  };
};
