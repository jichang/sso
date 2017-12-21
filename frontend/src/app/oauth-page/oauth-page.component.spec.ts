import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { OauthPageComponent } from './oauth-page.component';

describe('OauthPageComponent', () => {
  let component: OauthPageComponent;
  let fixture: ComponentFixture<OauthPageComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ OauthPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(OauthPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
