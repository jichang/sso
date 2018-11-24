import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ScopeCreatePageComponent } from './scope-create-page.component';

describe('ScopeCreatePageComponent', () => {
  let component: ScopeCreatePageComponent;
  let fixture: ComponentFixture<ScopeCreatePageComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ScopeCreatePageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ScopeCreatePageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
