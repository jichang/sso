import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Application, ApplicationModelService } from '../application-model.service';
import { Scope } from '../scope-model.service';
import { Authorization } from '../authorization-model.service';
import { session } from '../model';

interface OauthParams {
  client_id: string;
  redirect_uri: string;
  response_type: string;
  scope: string;
  state: string;
}

@Component({
  selector: 'oauth-page',
  templateUrl: './oauth-page.component.html',
  styleUrls: ['./oauth-page.component.css']
})
export class OauthPageComponent implements OnInit {
  authorization: Authorization;
  params: OauthParams;

  constructor(private route: ActivatedRoute, private router: Router, private http: HttpClient) {
    this.params = {
      client_id: this.route.snapshot.queryParams['client_id'],
      redirect_uri: this.route.snapshot.queryParams['redirect_uri'],
      response_type: this.route.snapshot.queryParams['response_type'],
      scope: this.route.snapshot.queryParams['scope'],
      state: this.route.snapshot.queryParams['state'],
    };

    this.authorization = {
      client_app: null,
      server_app: null,
      scope: null,
    };
  }

  queryApplication() {
    let params = new URLSearchParams();
    params.set('client_id', this.params.client_id);
    params.set('scope', this.params.scope);

    let headers = new HttpHeaders({
      'Content-Type': 'application/json',
      'Authorization': 'Bearer ' + window.localStorage.getItem('jwt')
    });
    let options = {
      headers: headers,
      search: params
    };

    let user = session.currUser();
    let apiUri = '/api/v1/authorizations/preview';

    this.http.get(apiUri, options)
      .subscribe((response: Authorization) => {
        this.authorization = response;
      });
  }

  ngOnInit() {
    this.queryApplication();
  }

  authorize() {
    let headers = new HttpHeaders({
      'Content-Type': 'application/json',
      'Authorization': 'Bearer ' + window.localStorage.getItem('jwt')
    });
    let options = {
      headers: headers,
    };

    let user = session.currUser();
    let apiUri = '/api/v1/users/' + user.id + '/authorizations';

    this.http.post(apiUri, this.params, options)
      .subscribe((response: any) => {
        window.location.href = this.params.redirect_uri + '?code=' + response.code + '&state=' + response.state;
      });
  }
}
