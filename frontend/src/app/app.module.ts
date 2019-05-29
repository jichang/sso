import * as cuid from "cuid";
import { BrowserModule } from "@angular/platform-browser";
import { NgModule, Injectable } from "@angular/core";
import { ReactiveFormsModule } from "@angular/forms";
import { HttpClientModule, HTTP_INTERCEPTORS } from "@angular/common/http";
import { ServiceWorkerModule } from "@angular/service-worker";
import { RouterModule, Routes } from "@angular/router";
import { BrowserAnimationsModule } from "@angular/platform-browser/animations";
import { registerLocaleData } from "@angular/common";
import localeZhHans from "@angular/common/locales/zh-Hans";

registerLocaleData(localeZhHans, "zh-Hans");

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
import { RolesPageComponent } from "./roles-page/roles-page.component";
import { GroupsPageComponent } from "./groups-page/groups-page.component";
import { GroupPageComponent } from "./group-page/group-page.component";
import { RolesListComponent } from "./roles-list/roles-list.component";
import { RoleModelService } from "./role-model.service";
import { PermissionModelService } from "./permission-model.service";
import {
  PermissionPipe,
  ResourceTypePipe,
  ActionTypePipe
} from "./permission.pipe";
import { PasswordPageComponent } from "./password-page/password-page.component";
import { PasswordFormComponent } from "./password-form/password-form.component";
import { GroupsListComponent } from "./groups-list/groups-list.component";
import { GroupModelService } from "./group-model.service";
import { UsersPageComponent } from "./users-page/users-page.component";
import { UsersListComponent } from "./users-list/users-list.component";
import { TwoFaPageComponent } from "./two-fa-page/two-fa-page.component";
import { QrcodeComponent } from "./qrcode/qrcode.component";
import { TotpFormComponent } from "./totp-form/totp-form.component";
import { environment } from "../environments/environment";
import { NotFoundPageComponent } from "./not-found-page/not-found-page.component";
import { InvitationsPageComponent } from "./invitations-page/invitations-page.component";
import { InvitationsListComponent } from "./invitations-list/invitations-list.component";
import { InvitationModelService } from "./invitation-model.service";

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
    path: "password",
    component: PasswordPageComponent
  },
  {
    path: "2fa",
    component: TwoFaPageComponent
  },
  {
    path: "authorizations",
    component: AuthorizationsPageComponent
  },
  {
    path: "roles",
    component: RolesPageComponent
  },
  {
    path: "groups",
    component: GroupsPageComponent
  },
  {
    path: "groups/:id",
    component: GroupPageComponent
  },
  {
    path: "users",
    component: UsersPageComponent
  },
  {
    path: "oauth",
    component: OauthPageComponent
  },
  {
    path: "invitations",
    component: InvitationsPageComponent
  },
  {
    path: "**",
    component: NotFoundPageComponent
  }
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
    ScopeCreatePageComponent,
    RolesPageComponent,
    GroupsPageComponent,
    GroupPageComponent,
    RolesListComponent,
    PermissionPipe,
    ResourceTypePipe,
    ActionTypePipe,
    PasswordPageComponent,
    PasswordFormComponent,
    GroupsListComponent,
    UsersPageComponent,
    UsersListComponent,
    TwoFaPageComponent,
    QrcodeComponent,
    TotpFormComponent,
    NotFoundPageComponent,
    InvitationsPageComponent,
    InvitationsListComponent
  ],
  imports: [
    BrowserModule,
    ReactiveFormsModule,
    HttpClientModule,
    RouterModule.forRoot(routes),
    ServiceWorkerModule,
    BrowserAnimationsModule,
    FeblrMaterialModule,
    ServiceWorkerModule.register("ngsw-worker.js", {
      enabled: environment.production
    })
  ],
  entryComponents: [ConfirmDialogComponent],
  providers: [
    { provide: PermissionModelService, useClass: PermissionModelService },
    { provide: RoleModelService, useClass: RoleModelService },
    { provide: ApplicationModelService, useClass: ApplicationModelService },
    { provide: ContactModelService, useClass: ContactModelService },
    { provide: ScopeModelService, useClass: ScopeModelService },
    { provide: GroupModelService, useClass: GroupModelService },
    { provide: TokenModelService, useClass: TokenModelService },
    { provide: AuthorizationModelService, useClass: AuthorizationModelService },
    {
      provide: HTTP_INTERCEPTORS,
      useClass: TokenInterceptorService,
      multi: true
    },
    {
      provide: InvitationModelService,
      useClass: InvitationModelService
    }
  ],
  bootstrap: [AppComponent]
})
export class AppModule {}
