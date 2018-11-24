import * as cuid from "cuid";
import { BrowserModule } from "@angular/platform-browser";
import { NgModule, Injectable } from "@angular/core";
import { ReactiveFormsModule } from "@angular/forms";
import { HttpClientModule, HTTP_INTERCEPTORS } from "@angular/common/http";
import { ServiceWorkerModule } from "@angular/service-worker";
import {
  RequestOptions,
  RequestOptionsArgs,
  BaseRequestOptions
} from "@angular/http";
import { RouterModule, Routes } from "@angular/router";
import { BrowserAnimationsModule } from "@angular/platform-browser/animations";

import { MaskPipe } from "./mask.pipe";

import { ApplicationModelService } from "./application-model.service";
import { ScopeModelService } from "./scope-model.service";
import { ContactModelService } from "./contact-model.service";
import { AuthorizationModelService } from "./authorization-model.service";

import { AppComponent } from "./app.component";
import { HomePageComponent } from "./home-page/home-page.component";
import { SignupFormComponent } from "./signup-form/signup-form.component";
import { SignupPageComponent } from "./signup-page/signup-page.component";
import { SigninFormComponent } from "./signin-form/signin-form.component";
import { SigninPageComponent } from "./signin-page/signin-page.component";
import { VerifyFormComponent } from "./verify-form/verify-form.component";
import { VerifyPageComponent } from "./verify-page/verify-page.component";
import { ContactFormComponent } from "./contact-form/contact-form.component";
import { ContactsPageComponent } from "./contacts-page/contacts-page.component";
import { ProfileFormComponent } from "./profile-form/profile-form.component";
import { ApplicationFormComponent } from "./application-form/application-form.component";
import { ApplicationsPageComponent } from "./applications-page/applications-page.component";
import { ApplicationsListComponent } from "./applications-list/applications-list.component";
import { ApplicationPageComponent } from "./application-page/application-page.component";
import { ScopesListComponent } from "./scopes-list/scopes-list.component";
import { ScopeFormComponent } from "./scope-form/scope-form.component";
import { OauthPageComponent } from "./oauth-page/oauth-page.component";
import { DateControlValueAccessorDirective } from "./date-control-value-accessor.directive";
import { FormControlErrorsComponent } from "./form-control-errors/form-control-errors.component";
import { FormControlErrorComponent } from "./form-control-error/form-control-error.component";
import { CopyTextSpanComponent } from "./copy-text-span/copy-text-span.component";
import { ApplicationCreatePageComponent } from "./application-create-page/application-create-page.component";
import { ApplicationBasicComponent } from "./application-basic/application-basic.component";
import { ApplicationScopesComponent } from "./application-scopes/application-scopes.component";
import { SettingsPageComponent } from "./settings-page/settings-page.component";
import { ContactCreatePageComponent } from "./contact-create-page/contact-create-page.component";
import { ContactsListComponent } from "./contacts-list/contacts-list.component";
import { AuthorizationsPageComponent } from "./authorizations-page/authorizations-page.component";
import { AuthorizationsListComponent } from "./authorizations-list/authorizations-list.component";
import { ContactStatePipe } from "./contact-state.pipe";
import { SummaryPanelComponent } from "./summary-panel/summary-panel.component";
import { FeblrMaterialModule } from "./materal.module";
import { ConfirmDialogComponent } from "./confirm-dialog/confirm-dialog.component";
import { TokenInterceptorService } from "./token-interceptor.service";
import { ProfilePageComponent } from "./profile-page/profile-page.component";
import { ScopeCreatePageComponent } from "./scope-create-page/scope-create-page.component";
import { TokenModelService } from "./token-model.service";

@Injectable()
export class TrackableHttpOptions extends BaseRequestOptions {
  constructor() {
    super();
    this.headers.append("Content-Type", "application/json");
  }

  merge(options?: RequestOptionsArgs): RequestOptions {
    var newOptions = super.merge(options);
    newOptions.headers.set("X-Requested-Id", cuid());
    return newOptions;
  }
}

const routes: Routes = [
  { path: "", component: HomePageComponent },
  { path: "signup", component: SignupPageComponent },
  { path: "signin", component: SigninPageComponent },
  {
    path: "applications",
    component: ApplicationsPageComponent
  },
  {
    path: "applications/create",
    component: ApplicationCreatePageComponent
  },
  {
    path: "applications/:id",
    component: ApplicationPageComponent
  },
  {
    path: "applications/:id/scopes/create",
    component: ScopeCreatePageComponent
  },
  {
    path: "contacts",
    component: ContactsPageComponent
  },
  {
    path: "contacts/create",
    component: ContactCreatePageComponent
  },
  {
    path: "settings",
    component: SettingsPageComponent
  },
  {
    path: "profile",
    component: ProfilePageComponent
  },
  {
    path: "authorizations",
    component: AuthorizationsPageComponent
  },
  { path: "oauth", component: OauthPageComponent }
];

@NgModule({
  declarations: [
    MaskPipe,
    AppComponent,
    HomePageComponent,
    SignupFormComponent,
    SigninFormComponent,
    VerifyFormComponent,
    SignupPageComponent,
    SigninPageComponent,
    VerifyPageComponent,
    ContactFormComponent,
    ContactsPageComponent,
    ProfileFormComponent,
    ProfilePageComponent,
    ApplicationFormComponent,
    ApplicationsPageComponent,
    ApplicationsListComponent,
    ApplicationPageComponent,
    ScopesListComponent,
    ScopeFormComponent,
    OauthPageComponent,
    DateControlValueAccessorDirective,
    FormControlErrorsComponent,
    FormControlErrorComponent,
    CopyTextSpanComponent,
    ApplicationCreatePageComponent,
    ApplicationBasicComponent,
    ApplicationScopesComponent,
    SettingsPageComponent,
    ContactCreatePageComponent,
    ContactsListComponent,
    AuthorizationsPageComponent,
    AuthorizationsListComponent,
    ContactStatePipe,
    SummaryPanelComponent,
    ConfirmDialogComponent,
    ScopeCreatePageComponent
  ],
  imports: [
    BrowserModule,
    ReactiveFormsModule,
    HttpClientModule,
    RouterModule.forRoot(routes),
    ServiceWorkerModule,
    BrowserAnimationsModule,
    FeblrMaterialModule
  ],
  entryComponents: [ConfirmDialogComponent],
  providers: [
    { provide: RequestOptions, useClass: TrackableHttpOptions },
    { provide: ApplicationModelService, useClass: ApplicationModelService },
    { provide: ContactModelService, useClass: ContactModelService },
    { provide: ScopeModelService, useClass: ScopeModelService },
    { provide: TokenModelService, useClass: TokenModelService },
    { provide: AuthorizationModelService, useClass: AuthorizationModelService },
    {
      provide: HTTP_INTERCEPTORS,
      useClass: TokenInterceptorService,
      multi: true
    }
  ],
  bootstrap: [AppComponent]
})
export class AppModule {}
