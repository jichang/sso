import { Injectable } from "@angular/core";
import {
  HttpInterceptor,
  HttpHandler,
  HttpRequest,
  HttpErrorResponse
} from "@angular/common/http";
import { tap } from "rxjs/operators";
import { Router, RouteReuseStrategy, ActivatedRoute } from "@angular/router";

@Injectable({
  providedIn: "root"
})
export class TokenInterceptorService implements HttpInterceptor {
  constructor(private router: Router, private route: ActivatedRoute) {}

  intercept(request: HttpRequest<any>, next: HttpHandler) {
    const jwtToken = window.localStorage.getItem("jwt");
    if (jwtToken) {
      request = request.clone({
        setHeaders: {
          Authorization: `Bearer ${jwtToken}`
        }
      });
    }

    return next
      .handle(request)
      .pipe(tap(() => {}, err => this.handleError(err)));
  }

  handleError(err: HttpErrorResponse) {
    if (err.status === 401) {
      let redirectUrl = this.router.url;
      this.router.navigate(["/signin"], {
        queryParams: {
          redirectUrl
        }
      });
    }
  }
}
