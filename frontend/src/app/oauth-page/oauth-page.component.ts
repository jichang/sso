import { Component, OnInit } from "@angular/core";
import { ActivatedRoute, Router, DefaultUrlSerializer } from "@angular/router";
import { HttpClient, HttpHeaders, HttpParams } from "@angular/common/http";
import {
  Application,
  ApplicationModelService
} from "../application-model.service";
import { Scope } from "../scope-model.service";
import { Authorization } from "../authorization-model.service";
import { session } from "../model";

interface OauthParams {
  server_id: string;
  client_id: string;
  redirect_uri: string;
  response_type: string;
  scope: string;
  state: string;
}

@Component({
  selector: "oauth-page",
  templateUrl: "./oauth-page.component.html",
  styleUrls: ["./oauth-page.component.css"]
})
export class OauthPageComponent implements OnInit {
  authorization: Authorization;
  params: OauthParams;

  constructor(
    private route: ActivatedRoute,
    private router: Router,
    private http: HttpClient
  ) {
    this.params = {
      server_id: this.route.snapshot.queryParams["server_id"],
      client_id: this.route.snapshot.queryParams["client_id"],
      redirect_uri: this.route.snapshot.queryParams["redirect_uri"],
      response_type: this.route.snapshot.queryParams["response_type"],
      scope: this.route.snapshot.queryParams["scope"],
      state: this.route.snapshot.queryParams["state"]
    };

    this.authorization = {
      client_app: null,
      server_app: null,
      scope: null
    };
  }

  queryApplication() {
    let params = new HttpParams()
      .set("server_id", this.params.server_id)
      .set("client_id", this.params.client_id)
      .set("scope", this.params.scope);

    let headers = new HttpHeaders({
      "Content-Type": "application/json",
      Authorization: "Bearer " + window.localStorage.getItem("jwt")
    });
    let options = {
      headers: headers,
      params: params
    };

    let user = session.currUser();
    let apiUri = "/api/v1/authorizations/preview";

    this.http.get(apiUri, options).subscribe((response: Authorization) => {
      this.authorization = response;
    });
  }

  ngOnInit() {
    this.queryApplication();
  }

  authorize() {
    let headers = new HttpHeaders({
      "Content-Type": "application/json",
      Authorization: "Bearer " + window.localStorage.getItem("jwt")
    });
    let options = {
      headers: headers
    };

    let user = session.currUser();
    let apiUri = "/api/v1/users/" + user.id + "/authorizations";

    this.http.post(apiUri, this.params, options).subscribe((response: any) => {
      let redirectUri = new URL(this.params.redirect_uri);
      redirectUri.searchParams.append("code", response.code);
      redirectUri.searchParams.append("state", response.state);

      window.location.href = redirectUri.toString();
    });
  }
}
