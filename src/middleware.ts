import { i18nRouter } from 'next-i18n-router';
import { i18nConfig } from './app/i18n/i18nConfig';
import type { NextRequest } from 'next/server';


export const middleware = (request: NextRequest) => {
  // 使用 accept-language 标头检测用户的偏好语言
  return i18nRouter(request, i18nConfig);
}

// applies this middleware only to files in the app directory
export const config = {
  matcher: '/((?!api|static|.*\\..*|_next).*)'
};